use cissy::csvline::CsvLine;
use std::fs::File;
use std::io::Write;

#[test]
fn test_csvline() {
let txt = "hello";
let mut cline = CsvLine::new();
let mut output = Vec::new();

for _ in 0..25 {
cline.add_field(txt, 0, txt.len());
for field in &cline.field {
writeln!(output, "{:?}", field.data).unwrap();
}
}

assert_eq!(cline.get_field_count(), 25);
assert_eq!(cline.get_field(0), Some("hello"));

cline.reset();
for field in &cline.field {
writeln!(output, "{:?}", field.data).unwrap();
}

// Here you can compare `output` with expected output if needed
}

fn main() {}