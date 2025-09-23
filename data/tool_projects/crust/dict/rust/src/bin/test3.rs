use dict::dict::{
dict_create, dict_destroy, dict_serialize, dict_deserialize, dict_get, dict_len,
DictArgs, DictKeyAttr, DictValAttr, DictAlloc, DictType, Dict
};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write, BufRead, BufReader};
use std::mem::size_of;

/// Helper function mimicking #define dict_double(dict, key) (*(double*) dict_get(dict, key))
unsafe fn dict_double(dict: &mut Dict, key: i32) -> &mut f64 {
let data = dict_get(dict, &key.to_ne_bytes()).expect("dict_get returned None for double key");
let (prefix, middle, suffix) = data.align_to_mut::<f64>();
assert!(prefix.is_empty() && suffix.is_empty());
&mut middle[0]
}

#[test]
fn test3() {
// dict_args_t for i32 -> double
let dict_args = DictArgs {
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

// dict_t* dict1 = dict_create(dict_args);
let mut dict1 = dict_create(dict_args.clone());

// Insert 500 values
for i in 0..500 {
unsafe{
*dict_double(&mut dict1, i) = i as f64;
}
}

// Serialize
let mut size1 = 0;
let data1 = dict_serialize(&dict1, &mut size1);
if data1.is_none() {
eprintln!("Fail to serialize.");
std::process::exit(1);
}
let data1 = data1.unwrap();
dict_destroy(&mut dict1);

// Write to "test3.bin"
{
let mut file = File::create("test5.bin").expect("Unable to create file");
// The C code prints "%zu\n" (the size), so let's do that as well:
writeln!(file, "{}", size1).expect("Failed to write size");
file.write_all(&data1).expect("Failed to write data");
}

// Read from "test3.bin"
let mut file = File::open("test5.bin").expect("Unable to open file");
let mut buf_reader = BufReader::new(&mut file);

let mut size_line = String::new();
buf_reader.read_line(&mut size_line).expect("Failed to read size line");
let size2: usize = size_line.trim().parse().expect("Failed to parse size");

let mut data2 = vec![0u8; size2];
buf_reader.read_exact(&mut data2).expect("Failed to read data");

// Deserialize
let mut dict2 = dict_deserialize(dict_args.clone(), &data2);
// If dict2 == NULL in C, we do an error check
// Our Rust stub won't produce None, but we can check if it's some dummy condition in a real scenario.

// Print the values
let len = dict_len(&dict2);
for i in 0..(len as i32) {
    unsafe {
println!("[key]: {:2}, [val]: {:4.2}", i, *dict_double(&mut dict2, i));
    }
}

dict_destroy(&mut dict2);
}
fn main() {}