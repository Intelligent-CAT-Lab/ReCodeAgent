use crate::compiler::Pos;
use crate::vector::{
vector_back_ptr, vector_back_ptr_or_null, vector_create, vector_empty,
vector_pop, vector_push, vector_element_size, Vector,
};
use std::sync::Mutex;
use lazy_static::lazy_static;
#[derive(Debug, Default, Clone)]
pub struct NodeBinded {
pub owner: Option<Box<Node>>,
pub function: Option<Box<Node>>,
}
#[derive(Debug, Default, Clone)]
pub struct Node {
pub r#type: i32,
pub flags: i32,
pub pos: Pos,
pub binded: NodeBinded,
pub cval: Option<char>,
pub sval: Option<String>,
pub inum: Option<u32>,
pub lnum: Option<u64>,
pub llnum: Option<u64>,
}
/// Global list of all nodes. We store them here so we can push references (indices) into the vectors.
lazy_static! {
static ref NODES: Mutex<Vec<Node>> = Mutex::new(Vec::new());
}
/// Emulates `struct vector* node_vector`
lazy_static! {
static ref NODE_VECTOR: Mutex<Option<Vector>> = Mutex::new(None);
}
/// Emulates `struct vector* node_vector_root`
lazy_static! {
static ref NODE_VECTOR_ROOT: Mutex<Option<Vector>> = Mutex::new(None);
}
/// Converts a u64 index to 8 bytes LE.
fn encode_index(idx: u64) -> [u8; 8] {
idx.to_le_bytes()
}
/// Converts 8 bytes LE to u64.
fn decode_index(bytes: &[u8]) -> Option<u64> {
    unimplemented!()
}
/// Sets the two global vectors used for node push/pop.
pub fn node_set_vector(vec: Vector, root_vec: Vector) 
{
    unimplemented!()
}
/// Pushes a node onto node_vector.
pub fn node_push(node: &Node) {
    unimplemented!()
}
/// Returns the last node or None if empty (like node_peek_or_null).
pub fn node_peek_or_null() -> Option<Node> {
    unimplemented!()
}
/// Returns the last Node. If none, returns default. Equivalent to node_peek in the original C code.
pub fn node_peek() -> Node {
    unimplemented!()
}
/// Pops the last node. Also checks if the same index matches node_vector_root top, popping that too.
pub fn node_pop() -> Node {
    unimplemented!()
}
/// Creates a new node from a template node, pushing it onto node_vector and returning the clone.
pub fn node_create(template: &Node) -> Node {
    unimplemented!()
}