use crate::tisp::{Rec, Tsp, TspType, Val};
pub fn create_int(num: f64, den: f64) -> Val {
    unimplemented!()
}
pub fn create_dec(num: f64, den: f64) -> Val {
    unimplemented!()
}
pub fn create_rat(num: f64, den: f64) -> Val {
    unimplemented!()
}
pub fn mk_num(a: TspType, b: TspType, force: i32) -> fn(f64, f64) -> Val {
    unimplemented!()
}
pub fn prim_add(st: &mut Tsp, vars: &mut Rec, args: Val) -> Val {
    unimplemented!()
}
pub fn prim_sub(st: &mut Tsp, vars: &mut Rec, args: Val) -> Val {
    unimplemented!()
}
pub fn prim_mul(st: &mut Tsp, vars: &mut Rec, args: Val) -> Val {
    unimplemented!()
}
pub fn prim_div(st: &mut Tsp, vars: &mut Rec, args: Val) -> Val {
    unimplemented!()
}
pub fn prim_mod(st: &mut Tsp, vars: &mut Rec, args: Val) -> Val {
    unimplemented!()
}
pub fn prim_pow(st: &mut Tsp, vars: &mut Rec, args: Val) -> Val {
    unimplemented!()
}
pub fn prim_denominator(st: &mut Tsp, env: &mut Rec, args: Val) -> Val {
    unimplemented!()
}
pub fn tib_env_math(st: &mut Tsp) {
    unimplemented!()
}