use cissy::csvfield::CsvField;
use std::fs::File;
use std::io::Write;

#[test]
fn test_csvfield() {
let mut cfield = CsvField::new();
let mut output = Vec::new();

writeln!(output, "{:?}", cfield.data).unwrap();

let txt = "hello";
cfield.set(txt, 0, txt.len());
writeln!(output, "{:?}", cfield.data).unwrap();

let txt = "123";
cfield.set(txt, 0, txt.len());
writeln!(output, "{:?}", cfield.data).unwrap();

let txt = "123456789012345678901234567";
cfield.set(txt, 0, txt.len());
writeln!(output, "{:?}", cfield.data).unwrap();

let txt = "123";
cfield.set(txt, 0, txt.len());
cfield.append(txt, 0, txt.len());
writeln!(output, "{:?}", cfield.data).unwrap();

cfield.reset();
writeln!(output, "{:?}", cfield.data).unwrap();

// Here you can compare `output` with expected output if needed
}

fn main() {}