use crate::tisp::{Tsp, Val, Prim, Rec};
pub type MkFn = fn(&mut Tsp, &str) -> Val;
pub fn val_string(st: &mut Tsp, args: Val, mk_fn: MkFn) -> Val {
unimplemented!()
}
pub fn prim_Str(st: &mut Tsp, env: &mut Rec, args: Val) -> Val {
unimplemented!()
}
pub fn prim_Sym(st: &mut Tsp, env: &mut Rec, args: Val) -> Val {
unimplemented!()
}
pub fn prim_strlen(st: &mut Tsp, env: &mut Rec, args: Val) -> Val {
unimplemented!()
}
pub fn form_strformat(st: &mut Tsp, env: &mut Rec, args: Val) -> Val {
unimplemented!()
}
pub fn tib_env_string(st: &mut Tsp) {
unimplemented!()
}