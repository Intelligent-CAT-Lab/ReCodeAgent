/// Reallocates a vector of bytes to a new size.
/// In C, this was 'void* xrealloc(void* ptr, size_t size)'.
pub fn xrealloc(_data: Vec<u8>, _new_size: usize) -> Vec<u8> {
unimplemented!()
}
/// Allocates a new vector of bytes of the specified size.
/// In C, this was 'void* xmalloc(size_t size)'.
pub fn xmalloc(_size: usize) -> Vec<u8> {
unimplemented!()
}