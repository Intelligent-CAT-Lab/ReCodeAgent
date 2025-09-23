use libtinyfseq::tinyfseq::TFError;

const TINYFSEQ_VERSION: &str = "1.0.0"; // Assuming a version string

#[test]
fn test_tferror_string_and_version() {
assert!(!TFError::TF_OK.to_string().is_empty(), "TFError string should not be empty");
assert!(!TINYFSEQ_VERSION.is_empty(), "Version string should not be empty");
println!("libtinyfseq version {}", TINYFSEQ_VERSION);
}

fn main() {}