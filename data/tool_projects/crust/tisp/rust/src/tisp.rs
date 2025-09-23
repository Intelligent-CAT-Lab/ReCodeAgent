pub const TSP_REC_MAX_PRINT: usize = 64;
pub const TSP_SYM_CHARS: &str = "_!?@#$%&~*-";
pub const TSP_REC_FACTOR: usize = 2;
#[derive(Debug, Clone, Copy)]
pub enum TspType {
TspNone = 1 << 0,
TspNil = 1 << 1,
TspInt = 1 << 2,
TspDec = 1 << 3,
TspRatio = 1 << 4,
TspStr = 1 << 5,
TspSym = 1 << 6,
TspPrim = 1 << 7,
TspForm = 1 << 8,
TspFunc = 1 << 9,
TspMacro = 1 << 10,
TspPair = 1 << 11,
TspRec = 1 << 12,
}
pub const TSP_EXPR: u32 = TSP_NUM | TspType::TspSym as u32 | TspType::TspPair as u32;
pub const TSP_RATIONAL: u32 = TspType::TspInt as u32 | TspType::TspRatio as u32;
pub const TSP_OP_CHARS: &str = "_+-*/\\|=^<>.:";
pub const TSP_NUM: u32 = TSP_RATIONAL | TspType::TspDec as u32;
pub struct Entry {
pub key: String,
pub val: Val,
}
pub type Prim = fn(Tsp, Rec, Val) -> Val;
pub struct Rec {
pub size: i32,
pub cap: i32,
pub items: Vec<Entry>,
pub next: Option<Box<Rec>>,
}
pub struct Tsp {
pub file: String,
pub filec: usize,
pub none: Val,
pub nil: Val,
pub t: Val,
pub env: Rec,
pub strs: Rec,
pub syms: Rec,
pub libh: Vec<*mut std::ffi::c_void>,
pub libhc: usize,
}
pub struct Val {
pub t: TspType,
pub v: ValUnion,
}
pub enum ValUnion {
S(String),
N { num: f64, den: f64 },
Pr { name: String, pr: Prim },
F { name: String, args: Box<Val>, body: Box<Val>, env: Rec },
P { car: Box<Val>, cdr: Box<Val> },
R(Rec),
}
pub fn rec_add(rec: &mut Rec, key: &str, val: Val) {
unimplemented!()
}
pub fn mk_rat(num: i32, den: i32) -> Option<Val> {
unimplemented!()
}
pub fn mk_val(t: TspType) -> Val {
unimplemented!()
}
pub fn tsp_lstlen(v: &Val) -> i32 {
unimplemented!()
}
pub fn tisp_env_init(cap: usize) -> Tsp {
unimplemented!()
}
pub fn tib_env_os(st: &mut Tsp) {
unimplemented!()
}
pub fn read_num(st: &mut Tsp) -> Val {
unimplemented!()
}
pub fn entry_get<'a>(rec: &'a Rec, key: &'a str) -> Option<&'a Entry> {
unimplemented!()
}
pub fn tib_env_string(st: &mut Tsp) {
unimplemented!()
}
pub fn prepend_bt(st: &mut Tsp, env: &mut Rec, f: Val) {
unimplemented!()
}
pub fn rec_get(rec: &Rec, key: &str) -> Option<Val> {
unimplemented!()
}
pub fn tisp_env_add(st: &mut Tsp, key: &str, v: Val) {
unimplemented!()
}
pub fn mk_pair(a: Val, b: Val) -> Option<Val> {
unimplemented!()
}
pub fn read_pair(st: &mut Tsp, endchar: char) -> Option<Val> {
unimplemented!()
}
pub fn tisp_read_sexpr(st: &mut Tsp) -> Option<Val> {
unimplemented!()
}
pub fn is_sym(c: char) -> bool {
unimplemented!()
}
pub fn mk_sym(st: &mut Tsp, s: &str) -> Option<Val> {
unimplemented!()
}
pub fn frac_reduce(num: &mut i32, den: &mut i32) {
unimplemented!()
}
pub fn tisp_read_line(st: &mut Tsp, level: i32) -> Option<Val> {
unimplemented!()
}
pub fn mk_prim(t: TspType, pr: Prim, name: &str) -> Option<Val> {
unimplemented!()
}
pub fn isnum(str: &str) -> bool {
unimplemented!()
}
pub fn tsp_type_str(t: TspType) -> &'static str {
unimplemented!()
}
pub fn mk_str(st: &mut Tsp, s: &str) -> Option<Val> {
unimplemented!()
}
pub fn is_op(c: char) -> bool {
unimplemented!()
}
pub fn esc_str(s: &str, len: i32, do_esc: i32) -> String {
unimplemented!()
}
pub fn tib_env_core(st: &mut Tsp) {
unimplemented!()
}
pub fn skip_ws(st: &mut Tsp, skipnl: i32) {
unimplemented!()
}
pub fn rec_extend(rec: &mut Rec, args: Val, vals: Val) -> Rec {
unimplemented!()
}
pub fn hash(key: &str) -> u32 {
unimplemented!()
}
pub fn mk_rec(st: &mut Tsp, env: Rec, assoc: Val) -> Option<Val> {
unimplemented!()
}
pub fn tisp_read(st: &mut Tsp) -> Option<Val> {
unimplemented!()
}
pub fn mk_int(i: i32) -> Val {
unimplemented!()
}
pub fn tib_env_math(st: &mut Tsp) {
unimplemented!()
}
pub fn tisp_eval_list(st: &mut Tsp, env: &mut Rec, v: Val) -> Option<Val> {
unimplemented!()
}
pub fn read_sci(st: &mut Tsp, val: f64, isint: i32) -> Option<Val> {
unimplemented!()
}
pub fn read_int(st: &mut Tsp) -> i32 {
unimplemented!()
}
pub fn rec_new(cap: usize, next: Option<Box<Rec>>) -> Rec {
unimplemented!()
}
pub fn read_str(st: &mut Tsp, mk_fn: fn(&mut Tsp, &str) -> Val) -> Option<Val> {
unimplemented!()
}
pub fn read_sym(st: &mut Tsp, is_char: fn(char) -> bool) -> Option<Val> {
unimplemented!()
}
pub fn mk_dec(d: f64) -> Option<Val> {
unimplemented!()
}
pub fn tisp_eval_body(st: &mut Tsp, env: &mut Rec, v: Val) -> Option<Val> {
unimplemented!()
}
pub fn tib_env_io(st: &mut Tsp) {
unimplemented!()
}
pub fn tisp_read_sugar(st: &mut Tsp, v: Val) -> Option<Val> {
unimplemented!()
}
pub fn tisp_env_lib(st: &mut Tsp, lib: &str) {
unimplemented!()
}
pub fn mk_list(st: &mut Tsp, n: i32, args: Vec<Val>) -> Option<Val> {
unimplemented!()
}
pub fn vals_eq(a: &Val, b: &Val) -> bool {
unimplemented!()
}
pub fn esc_char(c: char) -> char {
unimplemented!()
}
pub fn read_sign(st: &mut Tsp) -> i32 {
unimplemented!()
}
pub fn tisp_print(f: &mut std::fs::File, v: &Val) {
unimplemented!()
}
pub fn eval_proc(st: &mut Tsp, env: &mut Rec, f: Val, args: Val) -> Option<Val> {
unimplemented!()
}
pub fn tisp_eval(st: &mut Tsp, v: Val) -> Option<Val> {
unimplemented!()
}
pub fn mk_func(t: TspType, name: &str, args: Val, body: Val, env: Rec) -> Option<Val> {
unimplemented!()
}
pub fn rec_grow(rec: &mut Rec) {
unimplemented!()
}