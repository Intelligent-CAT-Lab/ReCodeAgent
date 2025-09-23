#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SymSet {
    bits: [u8; 256 / 8],
}
impl SymSet {
    pub fn empty() -> Self {
        unimplemented!()
    }
    pub fn full() -> Self {
        unimplemented!()
    }
    pub fn contains(&self, c: u8) -> bool {
        unimplemented!()
    }
    pub fn insert(&mut self, c: u8) {
        unimplemented!()
    }
    pub fn invert(&mut self) {
        unimplemented!()
    }
    pub fn union_with(&mut self, other: &SymSet) {
        unimplemented!()
    }
    pub fn intersect_with(&mut self, other: &SymSet) {
        unimplemented!()
    }
    pub fn is_empty(&self) -> bool {
        unimplemented!()
    }
}
pub fn symset_fmt(set: &SymSet) -> String {
    unimplemented!()
}
#[derive(Clone, Debug)]
pub struct NState {
    pub label: SymSet,
    pub target: Option<usize>,
    pub epsilon0: Option<usize>,
    pub epsilon1: Option<usize>,
}
impl NState {
    pub fn new() -> Self {
        unimplemented!()
    }
}
#[derive(Clone, Debug)]
pub struct Nfa {
    pub states: Vec<NState>,
    pub initial: usize,
    pub final_: usize,
    pub complemented: bool,
}
impl Nfa {
    pub fn new_single() -> Self {
        unimplemented!()
    }
    pub fn len(&self) -> usize {
        unimplemented!()
    }
}
pub fn nfa_free(_nfa: Nfa) {}
pub fn dfa_free(_dfa: Dfa) {}
pub fn nfa_clone(orig: &Nfa) -> Nfa {
    unimplemented!()
}
pub fn nfa_concat(nfa1: &mut Nfa, mut nfa2: Nfa) {
    unimplemented!()
}
pub fn nfa_pad_initial(nfa: &mut Nfa) {
    unimplemented!()
}
pub fn nfa_pad_final(nfa: &mut Nfa) {
    unimplemented!()
}
pub fn nfa_uncomplement(nfa: &mut Nfa) -> Result<(), String> {
    unimplemented!()
}
pub fn nfa_dump(nfa: &Nfa) {
    unimplemented!()
}
#[derive(Clone)]
pub struct DState {
    pub transitions: [usize; 256],
    pub accepting: bool,
    pub terminating: bool,
    pub bitset: Vec<u8>,
}
impl std::fmt::Debug for DState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unimplemented!()
    }
}
#[derive(Clone, Debug)]
pub struct Dfa {
    pub states: Vec<DState>,
    pub initial: usize,
}
impl Dfa {
    pub fn new() -> Self {
        unimplemented!()
    }
    pub fn len(&self) -> usize {
        unimplemented!()
    }
}
pub fn dfa_serialize(dfa: &Dfa) -> Vec<u8> {
    unimplemented!()
}
pub fn dfa_deserialize(buf: &[u8]) -> Result<(Dfa, usize), String> {
    unimplemented!()
}
pub fn dfa_dump(dfa: &Dfa) {
    unimplemented!()
}
fn leb128_put(buf: &mut Vec<u8>, mut n: i32) {
    unimplemented!()
}
fn leb128_get(buf: &[u8], p: &mut usize) -> Result<i32, String> {
    unimplemented!()
}
pub fn ltre_parse(regex: &str) -> Result<Nfa, String> {
    unimplemented!()
}
pub fn ltre_fixed_string(s: &str) -> Nfa {
    unimplemented!()
}
pub fn ltre_partial(nfa: &mut Nfa) -> Result<(), String> {
    unimplemented!()
}
pub fn ltre_ignorecase(nfa: &mut Nfa) -> Result<(), String> {
    unimplemented!()
}
pub fn ltre_complement(nfa: &mut Nfa) {
    unimplemented!()
}
pub fn ltre_compile(nfa: Nfa) -> Dfa {
    unimplemented!()
}
fn find_or_create_dead(states: &mut Vec<DState>) -> usize {
    unimplemented!()
}
fn step_powerset(nfa: &Nfa, bitset: &[u8], chr: u8) -> Vec<u8> {
    unimplemented!()
}
fn epsilon_closure_vec(nfa: &Nfa, start: usize, nfa_size: usize) -> Vec<u8> {
    unimplemented!()
}
fn epsilon_closure_into(nfa: &Nfa, st_id: usize, bitset: &mut [u8]) {
    unimplemented!()
}
fn dfa_minimize(dfa: &mut Dfa, complemented: bool) {
    unimplemented!()
}
fn bitset_test(bs: &[u8], idx: usize) -> bool {
    unimplemented!()
}
fn bitset_set(bs: &mut [u8], idx: usize) {
    unimplemented!()
}
fn all_bitset_indices(bs: &[u8]) -> impl Iterator<Item = usize> + '_ {
    let mut out = Vec::new();
    for (byte_i, &b) in bs.iter().enumerate() {
        if b != 0 {
            for bit_i in 0..8 {
                if b & (1 << bit_i) != 0 {
                    out.push(byte_i * 8 + bit_i);
                }
            }
        }
    }
    out.into_iter()
}
pub fn ltre_matches(dfa: &Dfa, input: &[u8]) -> bool {
    unimplemented!()
}
pub fn ltre_matches_lazy(dfap: &mut Option<Dfa>, nfa: &Nfa, input: &[u8]) -> bool {
    unimplemented!()
}
pub fn ltre_uncompile(dfa: &Dfa) -> Nfa {
    unimplemented!()
}
pub fn ltre_decompile(dfa: &Dfa) -> String {
    unimplemented!()
}
struct ParseContext<'a> {
    chars: &'a [u8],
    pos: usize,
}
impl<'a> ParseContext<'a> {
    fn new(s: &'a str) -> Self {
        unimplemented!()
    }
    fn peek(&self) -> Option<u8> {
        unimplemented!()
    }
    fn next(&mut self) -> Option<u8> {
        unimplemented!()
    }
    fn is_eof(&self) -> bool {
        unimplemented!()
    }
    fn expect_char(&mut self) -> Result<u8, String> {
        unimplemented!()
    }
}
fn parse_regex(ctx: &mut ParseContext) -> Result<Nfa, String> {
    unimplemented!()
}
fn parse_term(ctx: &mut ParseContext) -> Result<Nfa, String> {
    unimplemented!()
}
fn parse_factor(ctx: &mut ParseContext) -> Result<Nfa, String> {
    unimplemented!()
}
fn parse_atom(ctx: &mut ParseContext) -> Result<Nfa, String> {
    unimplemented!()
}
fn parse_symset(ctx: &mut ParseContext) -> Result<SymSet, String> {
    unimplemented!()
}
fn union_inplace(a: &mut SymSet, b: &SymSet) {
    unimplemented!()
}
fn intersect_inplace(a: &mut SymSet, b: &SymSet) {
    unimplemented!()
}
fn digits_set() -> SymSet {
    unimplemented!()
}
fn not_digits_set() -> SymSet {
    unimplemented!()
}
fn spaces_set() -> SymSet {
    unimplemented!()
}
fn not_spaces_set() -> SymSet {
    unimplemented!()
}
fn wordchar_set() -> SymSet {
    unimplemented!()
}
fn not_wordchar_set() -> SymSet {
    unimplemented!()
}
fn parse_natural(ctx: &mut ParseContext) -> Result<u32, String> {
    unimplemented!()
}
fn shift_option(opt: &mut Option<usize>, offset: usize) {
    unimplemented!()
}