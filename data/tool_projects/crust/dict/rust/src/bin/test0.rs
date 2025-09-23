use dict::dict::{
dict_new, dict_destroy, dict_get, DictType, Dict
};

/// Helper function that mimics the C macro:
/// #define dict_double(dict, key) (*(double*) dict_get(dict, key))
unsafe fn dict_double<'a>(dict: &'a mut Dict, key: i32) -> &'a mut f64 {
    println!("{}", key);
let data = dict_get(dict, &key.to_ne_bytes()).expect("dict_get returned None for double key");
let (prefix, middle, suffix) = data.align_to_mut::<f64>();
assert!(prefix.is_empty() && suffix.is_empty());
&mut middle[0]
}

#[test]
fn test0() {
// Equivalent to: dict_t* dict = dict_new(DICT_I32, sizeof(int32_t), sizeof(double));
let mut dict = dict_new(DictType::I32, std::mem::size_of::<i32>(), std::mem::size_of::<f64>());

// for (int32_t i = 0; i < 30; i++)
// {
//     dict_double(dict, i) = (double)i;
// }
for i in 0..30 {
unsafe{
*dict_double(&mut dict, i) = i as f64;
}
}

// for (int32_t i = 0; i < 30; i++)
// {
//     printf("[key]: %2" PRId32 ", [val]: %4.2lf\n", i, dict_double(dict, i));
// }
for i in 0..30 {
unsafe{
println!("[key]: {:2}, [val]: {:4.2}", i, *dict_double(&mut dict, i));
}
}

// dict_destroy(dict);
dict_destroy(&mut dict);
}

fn main(){}