use tisp_proj::tisp;
use tisp_proj::string;
use tisp_proj::core;
use tisp_proj::os;
use tisp_proj::math;
use std::fs::File;
use std::io::{self, Write, Read};
use std::time::Instant;
const TESTS: &[(&str, Option<&str>)] = &[
("self", None),
("1", Some("1")),
("2", Some("2")),
("0", Some("0")),
("30", Some("30")),
("12", Some("12")),
("-4", Some("-4")),
("-083", Some("-83")),
("-0", Some("0")),
("+4", Some("4")),
("+123", Some("123")),
("12.0", Some("12.0")),
("08", Some("8")),
("+12.34", Some("12.34")),
(".34", Some("0.34")),
("2.", Some("2.0")),
("1e0", Some("1")),
("1E+0", Some("1")),
("1e-0", Some("1")),
("1E4", Some("10000")),
(".1e-4", Some("1e-05")),
("-5.0e006", Some("-5000000.0")),
("-5.E+16", Some("-5e+16")),
("-.05", Some("-0.05")),
("-.0", Some("-0.0")),
("-1.e6", Some("-1000000.0")),
("3/4", Some("3/4")),
("4/3", Some("4/3")),
("+1/2", Some("1/2")),
("2/1", Some("2")),
("8/+1", Some("8")),
("8/4", Some("2")),
("4/8", Some("1/2")),
("02384/7238", Some("1192/3619")),
("-1/2", Some("-1/2")),
("1/-2", Some("-1/2")),
("-6/3", Some("-2")),
("-6/-3", Some("2")),
("\"foo\"", Some("foo")),
("\"foo bar\"", Some("foo bar")),
("True", Some("True")),
("()", Some("Nil")),
("Nil", Some("Nil")),
("Void", Some("Void")),
// Add more test cases as needed
];
fn tisp_test(st: &mut tisp::Tsp, input: &str, expect: Option<&str>, output: bool) -> bool {
st.file = input.to_string();
st.filec = 0;
let mut v = tisp::tisp_read(st);
if v.is_none() {
return false;
}
v = tisp::tisp_eval(st, v.unwrap());
if v.is_none() {
if output {
println!();
}
return false;
}
let mut f = File::create("test.out").unwrap();
tisp::tisp_print(&mut f, &v.unwrap());
f.flush().unwrap();
let mut f = File::open("test.out").unwrap();
let mut buf = String::new();
f.read_to_string(&mut buf).unwrap();
std::fs::remove_file("test.out").unwrap();
if output {
println!("{}", buf);
}
expect.map_or(false, |e| buf.trim() == e)
}
#[test]
fn run_tests() {
let mut correct = 0;
let mut total = 0;
let mut seccorrect = 0;
let mut sectotal = 0;
let mut last = 0;
let mut errors = vec![false; TESTS.len()];
let start = Instant::now();
let mut st = tisp::tisp_env_init(1024);
tisp::tib_env_core(&mut st);
tisp::tib_env_math(&mut st);
tisp::tib_env_string(&mut st);
tisp::tisp_env_lib(&mut st, "tibs");
for (i, &(input, expect)) in TESTS.iter().enumerate() {
if expect.is_none() {
if i != 0 {
println!("{}/{}", seccorrect, sectotal);
for j in last..i {
if TESTS[j].1.is_none() {
println!("{:<10}", TESTS[j].0);
} else if errors[j] {
println!("  input: {}\n    expect: {}\n    output: ", TESTS[j].0, TESTS[j].1.unwrap());
tisp_test(&mut st, TESTS[j].0, TESTS[j].1, true);
}
}
last = i + 1;
}
if input.is_empty() {
break;
}
println!("{:<10} ", input);
seccorrect = 0;
sectotal = 0;
} else {
if tisp_test(&mut st, input, expect, false) {
correct += 1;
seccorrect += 1;
} else {
errors[i] = true;
}
total += 1;
sectotal += 1;
}
}
let duration = start.elapsed();
println!("{:<10} {}/{}", "total", correct, total);
println!("{:?} ms", duration.as_millis());
assert_eq!(correct, total, "Some tests failed");
}
fn main() {}