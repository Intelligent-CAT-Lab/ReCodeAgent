pub fn top_check_init(nb_vars: i32, check_model: bool, lenient: bool) {
unimplemented!()
}
pub fn top_check_commit_formula_sig(f_sig: &[u8]) {
unimplemented!()
}
pub fn top_check_validate_sat(model: &[i32], size: u64, out_signature_or_null: &mut Option<Vec<u8>>) -> bool {
unimplemented!()
}
pub fn top_check_delete(ids: &[u64], nb_ids: i32) -> bool {
unimplemented!()
}
pub fn top_check_end_load() -> bool {
unimplemented!()
}
pub fn top_check_import(id: u64, literals: &[i32], nb_literals: i32, signature_data: &[u8]) -> bool {
unimplemented!()
}
pub fn top_check_valid() -> bool {
unimplemented!()
}
pub fn top_check_load(lit: i32) {
unimplemented!()
}
pub fn compute_clause_signature(id: u64, lits: &[i32], nb_lits: i32, out: &mut [u8]) {
unimplemented!()
}
pub fn top_check_validate_unsat(out_signature_or_null: &mut Option<Vec<u8>>) -> bool {
unimplemented!()
}
pub fn top_check_produce(id: u64, literals: &[i32], nb_literals: i32, hints: &[u64], nb_hints: i32, out_sig_or_null: &mut Option<Vec<u8>>) -> bool {
unimplemented!()
}