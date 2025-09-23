use std::time::{Duration, Instant};
pub fn nanoseconds() -> u64 {
let now = Instant::now();
now.elapsed().as_nanos() as u64
}
pub fn main() {
    unimplemented!()
}