pub const CBMT_NODE_SIZE: usize = 4;
// pub const CBMT_NODE_SIZE: usize = 32;
pub const CBMT_ERROR_OVER_CAPACITY: i32 = -1;
pub const CBMT_ERROR_QUEUE_EMPTY: i32 = -2;
pub const CBMT_ERROR_PROOF_ROOT: i32 = -3;
pub const CBMT_ERROR_BUILD_PROOF: i32 = -4;
pub const CBMT_ERROR_INVALID_CAPACITY: i32 = -5;
pub const CBMT_ERROR_VERIFY_FAILED: i32 = -6;
pub const CBMT_FATAL_BUILD_PROOF: i32 = -99;
#[derive(Debug, Default)]
pub struct CbmtBuffer<'a> {
    pub data: &'a mut [u8],
    pub capacity: usize,
}
#[derive(Debug, Clone, Default)]
pub struct CbmtNode {
    pub bytes: [u8; CBMT_NODE_SIZE],
}
#[derive(Debug, Clone)]
pub struct CbmtIndices {
    pub values: Vec<u32>,
    pub capacity: usize,
}
#[derive(Debug, Clone)]
pub struct CbmtProof {
    pub indices: CbmtIndices,
    pub lemmas: Vec<CbmtNode>,
}
#[derive(Debug, Clone, Default)]
pub struct CbmtTree {
    pub nodes: Vec<CbmtNode>,
    pub length: usize,
    pub capacity: usize,
}
#[derive(Debug, Clone)]
pub struct CbmtLeaves {
    pub nodes: Vec<CbmtNode>,
}
#[derive(Debug)]
pub struct CbmtQueue<'a> {
    pub buffer: CbmtBuffer<'a>,
    pub width: usize,
    pub length: usize,
    pub capacity: usize,
    pub tail: usize,
    pub head: usize,
}
#[derive(Debug, Clone)]
pub struct CbmtNodePair {
    pub index: u32,
    pub node: CbmtNode,
}
// Type alias for the node merge function.
// (In an idiomatic Rust implementation you might use generics or traits.)
pub type CbmtNodeMergeFn<Ctx> = fn(ctx: &mut Ctx, left: &CbmtNode, right: &CbmtNode) -> CbmtNode;
pub fn cbmt_universal_swap(left: &mut [u8], right: &mut [u8], width: usize) {
    unimplemented!()
}
pub fn cbmt_simple_bubble_sort<T>(slice: &mut [T], cmp: fn(&T, &T) -> i32) {
    unimplemented!()
}
pub fn cbmt_uint32_reverse_cmp(left: &u32, right: &u32) -> i32 {
    unimplemented!()
}
pub fn cbmt_buffer_init<'a>(buffer: &mut CbmtBuffer<'a>, data: &'a mut [u8]) {
    unimplemented!()
}
pub fn cbmt_leaves_init(leaves: &mut CbmtLeaves, nodes: Vec<CbmtNode>) {
    unimplemented!()
}
pub fn cbmt_indices_init(indices: &mut CbmtIndices, values: Vec<u32>) {
    unimplemented!()
}
pub fn cbmt_queue_init<'a>(
    queue: &mut CbmtQueue<'a>,
    buffer: CbmtBuffer<'a>,
    width: usize,
    capacity: usize,
) -> i32 {
    unimplemented!()
}
pub fn cbmt_queue_push_back(queue: &mut CbmtQueue, item: &[u8]) -> i32 {
    unimplemented!()
}
pub fn cbmt_queue_push_front(queue: &mut CbmtQueue, item: &[u8]) -> i32 {
    unimplemented!()
}
pub fn cbmt_queue_pop_front(queue: &mut CbmtQueue, item: &mut [u8]) -> i32 {
    unimplemented!()
}
pub fn cbmt_queue_front<'a>(queue: &'a CbmtQueue<'a>) -> Option<&'a [u8]> {
    unimplemented!()
}
pub fn cbmt_node_copy(dest: &mut CbmtNode, src: &CbmtNode) {
    unimplemented!()
}
pub fn cbmt_node_cmp(left: &CbmtNode, right: &CbmtNode) -> i32 {
    unimplemented!()
}
pub fn cbmt_node_pair_reverse_cmp(left: &CbmtNodePair, right: &CbmtNodePair) -> i32 {
    unimplemented!()
}
pub fn cbmt_tree_build_proof(
    tree: &CbmtTree,
    leaf_indices: &CbmtIndices,
) -> Result<CbmtProof, i32> {
    unimplemented!()
}
pub fn cbmt_tree_root(tree: &CbmtTree) -> CbmtNode {
    unimplemented!()
}
pub fn cbmt_proof_root<Ctx>(
    proof: &CbmtProof,
    root: &mut CbmtNode,
    leaves: &CbmtLeaves,
    merge: CbmtNodeMergeFn<Ctx>,
    merge_ctx: &mut Ctx,
    nodes_buffer: CbmtBuffer,
    pairs_buffer: CbmtBuffer,
) -> i32 {
    unimplemented!()
}
pub fn cbmt_proof_verify(
    proof: &CbmtProof,
    expected_root: &CbmtNode,
    leaves: &CbmtLeaves,
    merge: fn(Option<&mut ()>, &CbmtNode, &CbmtNode) -> CbmtNode,
) -> i32 {
    unimplemented!()
}
pub fn cbmt_build_merkle_root(
    leaves: &CbmtLeaves,
    merge: fn(Option<&mut ()>, &CbmtNode, &CbmtNode) -> CbmtNode,
) -> Result<CbmtNode, i32> {
    unimplemented!()
}
pub fn cbmt_build_merkle_tree(
    tree: &mut CbmtTree,
    leaves: &CbmtLeaves,
    merge: fn(Option<&mut ()>, &CbmtNode, &CbmtNode) -> CbmtNode,
) -> i32 {
    unimplemented!()
}
pub fn cbmt_build_merkle_proof<Ctx>(
    proof: &mut CbmtProof,
    leaves: &CbmtLeaves,
    leaf_indices: &CbmtIndices,
    merge: CbmtNodeMergeFn<Ctx>,
    merge_ctx: &mut Ctx,
    nodes_buffer: CbmtBuffer,
    indices_buffer: CbmtBuffer,
    lemmas_buffer: CbmtBuffer,
) -> i32 {
    unimplemented!()
}