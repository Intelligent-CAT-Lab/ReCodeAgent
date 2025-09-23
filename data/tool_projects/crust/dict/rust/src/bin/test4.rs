use dict::dict::{
dict_create, dict_destroy, dict_get, dict_has, DictArgs, DictKeyAttr, DictValAttr, DictAlloc,
DictType, Dict
};
use std::mem::size_of;

/// Mimic #define dict_double(dict, key) (*(double*) dict_get(dict, key))
unsafe fn dict_double(dict: &mut Dict, key: i32) -> &mut f64 {
let data = dict_get(dict, &key.to_ne_bytes()).expect("dict_get returned None");
let (prefix, middle, suffix) = data.align_to_mut::<f64>();

assert!(prefix.is_empty() && suffix.is_empty());
&mut middle[0]
}

#[test]
fn test4() {
// dict_t* dict = dict_create((dict_args_t){ .key = { .type = DICT_I32 }, .val = { .size = sizeof(double) } });
let args = DictArgs {
key: DictKeyAttr {
type_: DictType::I32,
size: size_of::<i32>(),
copy: None,
free: None,
hash: None,
cmpr: None,
},
val: DictValAttr {
size: size_of::<f64>(),
free: None,
},
alloc: DictAlloc {
malloc: None,
free: None,
},
};
let mut dict = dict_create(args);

// Insert 500 values
for i in 0..500 {
    unsafe{
        *dict_double(&mut dict, i) = i as f64;
    }
}

// Check each with dict_has, then print the double
for i in 0..500 {
let has_key = dict_has(&dict, &i.to_string().as_bytes());
print!("{}: ", if has_key { "yes" } else { " no" });
unsafe {
println!("{:5.2}", *dict_double(&mut dict, i));
}
}

dict_destroy(&mut dict);
}
fn main(){}