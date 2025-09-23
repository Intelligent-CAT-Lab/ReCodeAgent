use crate::trusted_utils;
pub fn reset_assignments() {
unimplemented!()
}
pub fn lrat_check_add_clause(id: u64, lits: &[i32], nb_lits: i32, hints: &[u64], nb_hints: i32) -> bool {
unimplemented!()
}
pub fn lrat_check_add_axiomatic_clause(id: u64, lits: &[i32], nb_lits: i32) -> bool {
unimplemented!()
}
pub fn check_clause(base_id: u64, lits: &[i32], nb_lits: i32, hints: &[u64], nb_hints: i32) -> bool {
unimplemented!()
}
pub fn lrat_check_end_load(out_sig: &mut Option<Vec<u8>>) -> bool {
unimplemented!()
}
pub fn lrat_check_delete_clause(ids: &[u64], nb_ids: i32) -> bool {
unimplemented!()
}
pub fn clauses_equivalent(left_cls: &[i32], right_cls: &[i32]) -> bool {
unimplemented!()
}
pub fn lrat_check_validate_sat(model: &[i32], size: u64) -> bool {
unimplemented!()
}
pub fn lrat_check_load(lit: i32) -> bool {
unimplemented!()
}
pub fn lrat_check_init(nb_vars: i32, opt_check_model: bool, opt_lenient: bool) {
unimplemented!()
}
pub fn clause_init(data: &[i32], nb_lits: i32) -> Vec<i32> {
unimplemented!()
}
pub fn lrat_check_validate_unsat() -> bool {
unimplemented!()
}