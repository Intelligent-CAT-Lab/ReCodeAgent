// Converted from the original C test script content: "876"

use kairoCompiler::buffer;
use kairoCompiler::vector;
use kairoCompiler::compiler;

#[cfg(test)]
mod tests {
use super::*;

#[test]
fn test_876() {
    let res = compiler::compile_file("src/bin/test2.txt", "src/bin/output2.txt", 0);
    assert!(res == 0, "Compilation failed.");
}
}

fn main() {}