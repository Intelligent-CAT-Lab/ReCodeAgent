pub const FALSE: u8 = 0;
pub const TRUE: u8 = 1;
pub const UNSIGNED: u8 = 2;
pub const SIGNED: u8 = 3;
pub const INTEL: u8 = 4;
pub const MOTOROLA: u8 = 5;
/// Extract a signal value from a CAN frame.
///
/// * `frame`      – byte slice of the frame data.
/// * `startbit`   – signal start bit.
/// * `len`        – signal length (in bits).
/// * `signedness` – either UNSIGNED or SIGNED.
/// * `endianness` – either INTEL (little-endian) or MOTOROLA (big-endian).
///
/// Returns a u64 value (if the signal is signed, it is already sign‐extended).
pub fn extract(frame: &[u8], startbit: u16, len: u8, signedness: u8, endianness: u8) -> u64 {
    unimplemented!()
}
/// Insert a signal value into a CAN frame.
///
/// * `frame`     – mutable byte slice.
/// * `startbit`  – signal start bit.
/// * `len`       – signal length in bits.
/// * `can_value` – signal value (as raw bits).
/// * `endianness`– INTEL or MOTOROLA.
pub fn insert(frame: &mut [u8], startbit: u16, len: u8, can_value: u64, endianness: u8) {
    unimplemented!()
}
/// Encode an unsigned 64‑bit physical value into the CAN frame.
pub fn encode_uint64_t(
    frame: &mut [u8],
    phy_value: u64,
    startbit: u16,
    len: u8,
    endianness: u8,
    factor: f64,
    offset: f64,
) {
    unimplemented!()
}
/// Encode a signed 64‑bit physical value into the CAN frame.
pub fn encode_int64_t(
    frame: &mut [u8],
    phy_value: i64,
    startbit: u16,
    len: u8,
    endianness: u8,
    factor: f64,
    offset: f64,
) {
    unimplemented!()
}
/// Encode a double‑precision (f64) physical value.
pub fn encode_double(
    frame: &mut [u8],
    phy_value: f64,
    startbit: u16,
    len: u8,
    endianness: u8,
    factor: f64,
    offset: f64,
) {
    unimplemented!()
}
/// Encode a single‑precision (f32) physical value.
pub fn encode_float(
    frame: &mut [u8],
    phy_value: f32,
    startbit: u16,
    len: u8,
    endianness: u8,
    factor: f64,
    offset: f64,
) {
    unimplemented!()
}
/// Decode an unsigned 64‑bit value from the CAN frame.
pub fn decode_uint64_t(
    frame: &[u8],
    startbit: u16,
    len: u8,
    endianness: u8,
    factor: f64,
    offset: f64,
) -> u64 {
    unimplemented!()
}
/// Decode a signed 64‑bit value from the CAN frame.
pub fn decode_int64_t(
    frame: &[u8],
    startbit: u16,
    len: u8,
    endianness: u8,
    factor: f64,
    offset: f64,
) -> i64 {
    unimplemented!()
}
/// Decode a double‑precision (f64) physical value.
pub fn decode_double(
    frame: &[u8],
    startbit: u16,
    len: u8,
    endianness: u8,
    factor: f64,
    offset: f64,
) -> f64 {
    unimplemented!()
}
/// Decode a single‑precision (f32) physical value.
pub fn decode_float(
    frame: &[u8],
    startbit: u16,
    len: u8,
    endianness: u8,
    factor: f64,
    offset: f64,
) -> f32 {
    unimplemented!()
}