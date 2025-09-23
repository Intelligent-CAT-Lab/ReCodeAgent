/// Returns the number of elements in an array.
/// This is a Rust equivalent of the COUNTOF macro in C.
#[macro_export]
macro_rules! countof {
    ($array:expr) => {
        $array.len()
    };
}