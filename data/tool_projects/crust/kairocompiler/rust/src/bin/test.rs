// Converted from the original C test script content: "877"

use kairoCompiler::buffer;
use kairoCompiler::vector;
use kairoCompiler::compiler;

#[cfg(test)]
mod tests {
use super::*;

#[test]
fn test_877() {
// Demonstrate usage of 877 in a test
let res = compiler::compile_file("src/bin/test_file.txt", "src/bin/output.txt", 0);
assert!(res == 0, "Compilation failed.");
}
}

fn main() {}