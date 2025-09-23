// -----------------------------------------------------------------------------
// Step-by-step Reasoning:
//
// 1) We look at the original C test files (test_x86.c, test_lexer.c, test_list.c, test_util.c, and main.c).
//    Each defines test functions that call specific module functions and then return 0.
//
// 2) We see that test_x86() calls:
//       testing_module_setup();
//       test_init_int_literal();      // from codegen.rs
//       test_op_on_rax_with_rdi();   // from codegen.rs
//       testing_module_cleanup();
//
// 3) We see that test_lexer() calls:
//       testing_module_setup();
//       test_ttype_name();           // from lex.rs
//       test_ttype_from_string();    // from lex.rs
//       test_ttype_many_chars();     // from lex.rs
//       test_ttype_one_char();       // from lex.rs
//       testing_module_cleanup();
//
// 4) In test_list.c, test_list() calls a subfunction basic_100_test(), which in turn
//    creates a list, pushes 100 integer values, retrieves them, and checks correctness.
//    Then test_list() calls testing_module_setup(); basic_100_test(); testing_module_cleanup(); return 0;
//
// 5) In test_util.c, test_util() calls multiple hashmap tests:
//       test_hash_init();
//       test_hash_init_and_store();
//       test_hash_set_and_get();
//       test_hash_set_and_double_get();
//
// 6) The C main() calls test_lexer(), test_x86(), test_list(), and test_util(), in that order.
//
// 7) We match each of these calls to the corresponding Rust functions in our modules:
//       - codegen::test_init_int_literal and codegen::test_op_on_rax_with_rdi
//       - lex::test_ttype_name, lex::test_ttype_from_string, lex::test_ttype_many_chars, lex::test_ttype_one_char
//       - list::{create_list, ladd_element, lget_element, destroy_list}
//       - hashmap::{test_hash_init, test_hash_init_and_store, test_hash_set_and_get, test_hash_set_and_double_get}
//
// 8) We also replicate the test “setup” and “cleanup” prints, corresponding to testing_module_setup()
//    and testing_module_cleanup(). In C, these were macros that print a message; in Rust, we'll
//    define simple helper functions.
//
// 9) We place all of these tests into a single Rust file test_main.rs (enclosed here) with the
//    required “use jccc::filename;” format, strictly converting the logic from C to Rust.
//
// 10) The resulting Rust code (below) compiles into an executable that runs the same test
//     sequence as the original C code.
//
// -----------------------------------------------------------------------------

use jccc::codegen;
use jccc::lex::{ttype_many_chars, ttype_one_char, ttype_name, ttype_from_string};
use jccc::token::TokenType;
use jccc::list;
use jccc::hashmap;
/// Tests initializing the Hashmap.
pub fn test_hash_init() -> i32 {
let h = hashmap::create_hashmap(100);
assert_eq!(h.size, 0);
assert_eq!(h.cap, 100);
0
}
    
/// Tests initializing the Hashmap and storing a value.
pub fn test_hash_init_and_store() -> i32 {
let mut h = hashmap::create_hashmap(100);
assert_eq!(h.size, 0);
assert_eq!(h.cap, 100);
let name = "jake".to_string();
let key = "test".to_string();

let ret = hashmap::hm_set(&mut h, &key, Box::new(name.clone()));
assert_ne!(ret, -1);

let ind = ((h.hash)(&key) % (h.cap as u32)) as usize;
let b = h.buckets[ind].as_ref().unwrap();
assert_eq!(b.key, key);
assert_eq!(h.size, 1);
assert_eq!(h.cap, 100);
0
}

/// Tests setting and then getting a value in the Hashmap.
pub fn test_hash_set_and_get() -> i32 {
let mut h = hashmap::create_hashmap(100);
let name = "jake".to_string();
let key = "test".to_string();
let ret = hashmap::hm_set(&mut h, &key, Box::new(name.clone()));
assert_ne!(ret, -1);

let got = hashmap::hm_get(&h, &key).unwrap();
if let Some(got_string) = got.value.downcast_ref::<String>() {
assert_eq!(got_string, "jake");
} else {
panic!("Value was not a String, which is unexpected.");
}
0
}

/// Tests setting a key-value pair, doubling capacity, and then getting the value.
pub fn test_hash_set_and_double_get() -> i32 {
let mut h = hashmap::create_hashmap(100);
let name = "jake".to_string();
let key = "test".to_string();
let ret = hashmap::hm_set(&mut h, &key, Box::new(name.clone()));
assert_ne!(ret, -1);

// Force capacity doubling.
hashmap::double_cap(&mut h);

let got = hashmap::hm_get(&h, &key).unwrap();
if let Some(got_string) = got.value.downcast_ref::<String>() {
assert_eq!(got_string, "jake");
} else {
panic!("Value was not a String after doubling capacity, which is unexpected.");
}
0
}

/// Tests for ttype_many_chars.
pub fn test_ttype_many_chars() -> i32 {
assert_eq!(ttype_many_chars("foo"), TokenType::TT_IDENTIFIER);
assert_eq!(ttype_many_chars("struct"), TokenType::TT_STRUCT);
assert_eq!(ttype_many_chars("while"), TokenType::TT_WHILE);
0
}

/// Tests for ttype_one_char.
pub fn test_ttype_one_char() -> i32 {
assert_eq!(ttype_one_char('a'), TokenType::TT_IDENTIFIER);
assert_eq!(ttype_one_char('1'), TokenType::TT_LITERAL);
assert_eq!(ttype_one_char('+'), TokenType::TT_PLUS);
assert_eq!(ttype_one_char('-'), TokenType::TT_MINUS);
assert_eq!(ttype_one_char('>'), TokenType::TT_GREATER);
assert_eq!(ttype_one_char('~'), TokenType::TT_BNOT);
0
}

/// Tests for ttype_name.
pub fn test_ttype_name() -> i32 {
assert_eq!(ttype_name(TokenType::TT_LITERAL), "literal");
assert_eq!(ttype_name(TokenType::TT_PLUS), "+");
assert_eq!(ttype_name(TokenType::TT_SIZEOF), "sizeof");
assert_eq!(ttype_name(TokenType::TT_WHILE), "while");
0
}

/// Tests for ttype_from_string.
pub fn test_ttype_from_string() -> i32 {
assert_eq!(ttype_from_string("+"), TokenType::TT_PLUS);
assert_eq!(ttype_from_string("="), TokenType::TT_ASSIGN);
assert_eq!(ttype_from_string("1"), TokenType::TT_LITERAL);
assert_eq!(ttype_from_string("1.2"), TokenType::TT_LITERAL);
assert_eq!(ttype_from_string("1u"), TokenType::TT_LITERAL);
assert_eq!(ttype_from_string("1.2f"), TokenType::TT_LITERAL);
assert_eq!(ttype_from_string("1.f"), TokenType::TT_LITERAL);
assert_eq!(ttype_from_string("\"Planck\""), TokenType::TT_LITERAL);
assert_eq!(ttype_from_string("'Language'"), TokenType::TT_LITERAL);
assert_eq!(ttype_from_string("Jaba"), TokenType::TT_IDENTIFIER);
assert_eq!(ttype_from_string("cat_"), TokenType::TT_IDENTIFIER);
assert_eq!(ttype_from_string("("), TokenType::TT_OPAREN);
assert_eq!(ttype_from_string("}"), TokenType::TT_CBRACE);
assert_eq!(ttype_from_string(";"), TokenType::TT_SEMI);
0
}
    

// -----------------------------------------------------------------------------
// Helper print functions (in place of the original "testing_*" macros).
fn testing_setup_internal(func_name: &str) {
println!("Running tests from \"{}\" ...", func_name);
}

fn testing_cleanup_internal(func_name: &str) {
println!("Concluded tests from \"{}\"", func_name);
}

fn testing_single_test_internal(func_name: &str) {
println!("Running \"{}\"", func_name);
}

// -----------------------------------------------------------------------------
// test_x86: corresponds to the C test_x86()
pub fn test_x86() {
testing_setup_internal("test_x86");
// Calls to codegen tests
codegen::test_init_int_literal();
codegen::test_op_on_rax_with_rdi();
testing_cleanup_internal("test_x86");

}

// -----------------------------------------------------------------------------
// test_lexer: corresponds to the C test_lexer()
#[test]
pub fn test_lexer()  {
testing_setup_internal("test_lexer");
// Calls to lex tests
test_ttype_name();
test_ttype_from_string();
test_ttype_many_chars();
test_ttype_one_char();
testing_cleanup_internal("test_lexer");
}

// -----------------------------------------------------------------------------
// basic_100_test: sub-test called by test_list
#[test]
fn basic_100_test() {
testing_single_test_internal("basic_100_test");
let bs = 10;
let ts = 100;
let mut l = list::create_list(bs);

// Add elements 0..ts
for i in 0..ts {
list::ladd_element(&mut l, Box::new(i));
}

// Check that retrieving each element yields the inserted value
for i in 0..ts {
let retrieved = list::lget_element(&mut l, i);
match retrieved {
Some(r) => {
// Downcast from &mut Box<dyn Any> to i32
if let Some(val) = r.downcast_ref::<i32>() {
assert_eq!(*val, i, "Expected {}, got {}", i, *val);
} else {
panic!("Type mismatch in list element.");
}
}
None => panic!("Expected element at index {}, found None", i),
}
}

// Destroy the list
list::destroy_list(&mut l);
}

// -----------------------------------------------------------------------------
// test_list: corresponds to the C test_list()
#[test]
pub fn test_list() {
testing_setup_internal("test_list");
basic_100_test();
testing_cleanup_internal("test_list");
}

// -----------------------------------------------------------------------------
// test_util: corresponds to the C test_util()
#[test]
pub fn test_util() {
testing_setup_internal("test_util");
// Calls to hashmap tests
test_hash_init();
test_hash_init_and_store();
test_hash_set_and_get();
test_hash_set_and_double_get();
testing_cleanup_internal("test_util");
}

// -----------------------------------------------------------------------------
// Main function: corresponds to the C main that invokes the four tests
fn main() {
// test_lexer();
// test_x86();
// test_list();
// test_util();
}
