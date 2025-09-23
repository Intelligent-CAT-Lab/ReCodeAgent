use dict::dict::{
dict_create_args, dict_destroy, dict_get, dict_remove, DictType, DictArgs, DictKeyAttr,
DictValAttr, DictAlloc, Dict, DictDeepCopy, DictDestructor, DictHash, DictCmpr, dict_key
};
use std::mem::size_of;

/// This mimics the C struct:
/// typedef struct str {
///     size_t size;
///     char* str;
/// } str_t;
#[repr(C)]
#[derive(Debug, Clone)]
struct StrT {
size: usize,
// For simplicity, store data as a standard Rust String, though in the real C code
// it was a raw char* plus custom allocation.
// The dictionary is only storing bytes, so we are just illustrating usage.
// You would handle deeper ownership in a real scenario.
data: String,
}

/// In the C code, we have custom str_copy, str_free, etc. For Rust stubs, we'd rely on closures/fns
/// in the dictionary, but we're just demonstrating calls.
#[test]
fn test2() {
// Construct the DictKeyAttr that includes copy, free, hash, cmpr as in C
// Here, we simply store them as None to illustrate usage, because the actual logic is unimplemented.
let key_attr = DictKeyAttr {
type_: DictType::Struct,
size: size_of::<StrT>(),
copy: None,
free: None, // would be Some(fn) if we implement str_free
hash: None, // would be Some(fn) if we implement str_hash
cmpr: None, // would be Some(fn) if we implement str_cmpr
};
let val_attr = DictValAttr {
size: size_of::<u64>(),
free: None,
};
let alloc = DictAlloc { malloc: None, free: None };
let args = DictArgs {
key: key_attr,
val: val_attr,
alloc,
};

// dict_create_args( .key = key_attr, .val = { .size = sizeof(uint64_t) } )
let mut dict = dict_create_args(args);

// #define dict_uint64(dict, str) (*(uint64_t*) dict_get(dict, str))
// In C, we do: dict_uint64(dict, &str_t{size, s}) = some_value.

// We'll define a small utility to do that in Rust:
unsafe fn dict_uint64<'a>(dict: &'a mut Dict, key: &'a StrT) -> &'a mut u64 {
println!("{}", key.data);
let data = dict_get(dict, key.data.as_bytes()).expect("dict_get returned None for u64 key");
let (prefix, middle, suffix) = data.align_to_mut::<u64>();
assert!(prefix.is_empty() && suffix.is_empty());
&mut middle[0]
}

// Insert some values
let s1 = StrT { size: 2, data: "s1".to_string() };
let s2 = StrT { size: 2, data: "s2".to_string() };
let s3 = StrT { size: 2, data: "s3".to_string() };
let s4 = StrT { size: 2, data: "s4".to_string() };
let s5 = StrT { size: 2, data: "s5".to_string() };
unsafe{
*dict_uint64(&mut dict, &s1) = 1;
*dict_uint64(&mut dict, &s2) = 2;
*dict_uint64(&mut dict, &s3) = 3;
*dict_uint64(&mut dict, &s4) = 4;
*dict_uint64(&mut dict, &s5) = 5;
}

// printf("val: %lu\n", dict_uint64(dict, s4));
unsafe{
println!("val: {}", *dict_uint64(&mut dict, &s4));
}

// dict_remove(dict, &s3)
let removed = dict_remove(&mut dict, s3.data.as_bytes());
println!("remove: {}", removed);

// Retrieve keys with dict_key (unimplemented stub)
let mut size_of_keys = 0;
if let Some(_keys_bytes) = dict_key(&dict, &mut size_of_keys) {
println!("key amount: {}", size_of_keys);
println!("We would iterate and print each key, but the function is a stub in this demonstration.");
}

dict_destroy(&mut dict);
}
fn main() {}