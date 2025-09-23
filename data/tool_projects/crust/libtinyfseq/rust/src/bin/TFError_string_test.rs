use libtinyfseq::tinyfseq::TFError;

#[test]
fn test_tferror_to_string() {
for i in 0..=u8::MAX {
let error = match i {
0 => TFError::TF_OK,
1 => TFError::TF_EINVALID_MAGIC,
2 => TFError::TF_EINVALID_COMPRESSION_TYPE,
3 => TFError::TF_EINVALID_BUFFER_SIZE,
4 => TFError::TF_EINVALID_VAR_SIZE,
_ => continue, // Skip invalid values
};
let msg = error.to_string();
assert!(!msg.is_empty(), "TFError string should not be empty");
}
}

fn main() {}