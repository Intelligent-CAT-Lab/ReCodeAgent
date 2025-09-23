use SlothLang::parser::{parse, free_program};
use SlothLang::slothvm::execute;

#[test]
fn test_parser_hello_world() {
let mut program = parse("src/bin/Examples/HelloWorld.sloth");
println!("TESTING PARSER (1/3)");
assert_eq!(program.as_ref().unwrap().codes[0], 1);
assert_eq!(program.as_ref().unwrap().codes[1], 3);
assert_eq!(program.as_ref().unwrap().codes[2], 1);
assert_eq!(program.as_ref().unwrap().codes[14], 8);
assert_eq!(program.as_ref().unwrap().codes[17], 10);
free_program(program);
println!("----------------------------");
}

#[test]
fn test_parser_gamut() {
let mut program = parse("src/bin/Examples/Gamut.sloth");
println!("TESTING PARSER (2/3)");

const NUM_CODES: usize = 11;
let mut histogram = [0; NUM_CODES];
let mut i = 0;
let mut pc = program.as_ref().unwrap().codes[i];
while pc != 0 {
if pc > 0 && pc < NUM_CODES as u8 {
histogram[pc as usize] += 1;
}
i += 1;
pc = program.as_ref().unwrap().codes[i];
}

for i in 1..NUM_CODES {
assert!(histogram[i] > 0);
}
free_program(program);
println!("PARSER TEST PASSED");
println!("----------------------------");
}

#[test]
fn test_program_execution_count() {
let mut program = parse("src/bin/Examples/Count.sloth");
println!("TESTING PROGRAM EXECUTION (3/3)");
let x = execute(&mut program);
println!("Returned: {}", x);
assert_eq!(x, 11);
println!("EXECUTION TEST PASSED");
println!("----------------------------");
free_program(program);
}

fn main() {}