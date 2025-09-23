// Constants
pub const FNV_PRIME: u64 = 1099511628211;
pub const FNV_BASE: u64 = 14695981039346656037;
pub const HAMT_NODE_T_FLAG: u32 = 1;
pub const KEY_VALUE_T_FLAG: u32 = 0;
pub const CHUNK_SIZE: usize = 6;
// Type Definitions
pub type HashFn<T> = fn(&mut T) -> u32;
pub type EqualsFn<T> = fn(&mut T, &mut T) -> bool;
pub type StrFn<T> = fn(&mut T) -> String;
pub type DeallocateFn<T> = fn(&mut T);
pub struct KeyValue<'a, T, U> {
    pub key: &'a mut T,
    pub value: &'a mut U,
}
impl <T, U> KeyValue<'_, T, U> {
}
pub enum HamtNode<'a, T, U> {
    Leaf(Option<KeyValue<'a, T, U>>),
    Sub(SubNode<'a, T, U>),
}
impl <T, U> HamtNode<'_, T, U> {
    pub fn get_children_pointer(&mut self) -> Self {
        unimplemented!()
    }
    pub fn is_leaf(&mut self) -> bool {
        unimplemented!()
    }
    pub fn hamt_node_search(&mut self, hash: u32, lvl: i32, key: &mut T, equals_fn: EqualsFn<T>) -> Self {
        unimplemented!()
    }
    pub fn hamt_node_insert(&mut self, hash: u32, lvl: i32, key: &mut T, 
        value: &mut U, hash_fn: HashFn<T>, equals_fn: EqualsFn<T>, conflict_kv: &mut KeyValue<'_, T, U>) -> bool {
        unimplemented!()
    }
    pub fn hamt_node_remove(&mut self, hash: u32, lvl: i32, key: &mut T, equals_fn: EqualsFn<T>, removed_kv: &mut KeyValue<'_, T, U>) -> bool {
        unimplemented!()
    }
    pub fn hamt_node_destroy(&mut self, deallocate_fn_key: DeallocateFn<T>, deallocate_fn_val: DeallocateFn<U>) {
        unimplemented!()
    }
    pub fn hamt_node_print(&mut self, lvl:i32, str_fn_key: StrFn<T>, str_fn_val: StrFn<U>) {
        unimplemented!()
    }
}
pub struct SubNode<'a, T, U> {
    pub bitmap: u32,
    pub children: Option<Box<HamtNode<'a, T, U>>>,
}
pub struct Hamt<'a, T, U> {
    pub root: Option<Box<HamtNode<'a, T, U>>>,
    pub size: i32,
    pub hash_fn: HashFn<T>,
    pub equals_fn: EqualsFn<T>,
}
impl <'a, T, U> Hamt<'a, T, U> {
    pub fn new_hamt(hash_fn: HashFn<T>, equals_fn: EqualsFn<T>) -> Self {
        unimplemented!()
    }
    pub fn hamt_size(&self) -> i32 {
        unimplemented!()
    }
    pub fn hamt_set(&mut self, key: &mut T, value: &mut U, conflict_kv: &mut KeyValue<T, U>) -> bool {
        unimplemented!()
    }
    pub fn hamt_search(&self, key: &mut T) -> Option<KeyValue<'a, T, U>> {
        unimplemented!()
    }
    pub fn hamt_remove(&self, key: &mut T, removed_kv: &mut KeyValue<'a, T, U>) -> bool {
        unimplemented!()
    }
    pub fn hamt_destroy(&self, deallocate_fn: DeallocateFn<T>, deallocate_fn_val: DeallocateFn<U>) {
        unimplemented!()
    }
    pub fn hamt_print(&self, str_fn_key: StrFn<T>, str_fn_val: StrFn<U>) {
        unimplemented!()
    }
}
// Function Declarations
pub fn hamt_int_hash<T>(key: &mut T) -> u32 {
    unimplemented!()
}
pub fn hamt_str_hash<T>(key: &mut T) -> u32 {
    unimplemented!()
}
pub fn hamt_int_equals<T>(a: &mut T, b: &mut T) -> bool {
    unimplemented!()
}
pub fn hamt_str_equals<T>(a: &mut T, b: &mut T) -> bool {
    unimplemented!()
}
pub fn hamt_fnv1_hash<T>(key: &mut T, len: usize) {
    unimplemented!()
}
pub fn hamt_get_symbol(hash: u32, lvl: i32) {
    unimplemented!()
}