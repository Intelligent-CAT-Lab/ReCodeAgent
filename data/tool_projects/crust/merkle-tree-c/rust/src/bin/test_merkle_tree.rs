use merkle_tree_c::merkle_tree::*;

/// Converts an i32 value into a CbmtNode (storing the value in little-endian form).
fn int32_to_node(value: i32) -> CbmtNode {
    let mut node = CbmtNode {
        bytes: [0; CBMT_NODE_SIZE],
    };
    // Here CBMT_NODE_SIZE is 4 (node_i32 mode)
    node.bytes
        .copy_from_slice(&value.to_le_bytes()[..CBMT_NODE_SIZE]);
    node
}

/// Extracts an i32 value from a CbmtNode.
fn node_to_int32(node: &CbmtNode) -> i32 {
    i32::from_le_bytes(node.bytes)
}

/// Concrete implementation of the merge function for node_i32 mode.
/// Computes: merge(left, right) = (right - left) as an i32 stored in a node.
fn node_merge(_merge_ctx: Option<&mut ()>, left: &CbmtNode, right: &CbmtNode) -> CbmtNode {
    let left_value = node_to_int32(left);
    let right_value = node_to_int32(right);
    let value = right_value.wrapping_sub(left_value);
    int32_to_node(value)
}

#[test]
fn test_build_empty() {
    let leaves = CbmtLeaves { nodes: vec![] };
    let mut tree = CbmtTree {
        nodes: Vec::new(),
        length: 0,
        capacity: 0,
    };
    let ret = cbmt_build_merkle_tree(&mut tree, &leaves, node_merge);
    assert_eq!(ret, 0);
    assert_eq!(node_to_int32(&cbmt_tree_root(&tree)), 0);
    println!("test_build_empty passed.");
}

#[test]
fn test_build_five() {
    let leaves = CbmtLeaves {
        nodes: vec![
            int32_to_node(2),
            int32_to_node(3),
            int32_to_node(5),
            int32_to_node(7),
            int32_to_node(11),
        ],
    };
    let mut tree = CbmtTree {
        nodes: Vec::new(),
        length: 0,
        capacity: 0,
    };
    let ret = cbmt_build_merkle_tree(&mut tree, &leaves, node_merge);
    assert_eq!(ret, 0);
    for (i, node) in tree.nodes.iter().enumerate() {
        println!("tree.nodes[{}]: {}", i, node_to_int32(node));
    }
    // Expected values computed from the C algorithm:
    // tree.nodes[0] = 4, tree.nodes[1] = -2, tree.nodes[2] = 2, tree.nodes[3] = 4,
    // leaves: tree.nodes[4]=2, [5]=3, [6]=5, [7]=7, [8]=11.
    assert_eq!(node_to_int32(&tree.nodes[0]), 4);
    assert_eq!(node_to_int32(&tree.nodes[1]), -2);
    assert_eq!(node_to_int32(&tree.nodes[2]), 2);
    assert_eq!(node_to_int32(&tree.nodes[3]), 4);
    assert_eq!(node_to_int32(&tree.nodes[4]), 2);
    assert_eq!(node_to_int32(&tree.nodes[5]), 3);
    assert_eq!(node_to_int32(&tree.nodes[6]), 5);
    assert_eq!(node_to_int32(&tree.nodes[7]), 7);
    assert_eq!(node_to_int32(&tree.nodes[8]), 11);
    println!("test_build_five passed.");
}

#[test]
fn test_build_root_directly_2leaves() {
    let leaves = CbmtLeaves {
        nodes: vec![int32_to_node(2), int32_to_node(3)],
    };
    let root = cbmt_build_merkle_root(&leaves, node_merge).expect("Failed to build root");
    println!("root: {}", node_to_int32(&root));
    // For two leaves, merge(2,3) = 3 - 2 = 1.
    assert_eq!(node_to_int32(&root), 1);
    println!("test_build_root_directly_2leaves passed.");
}

#[test]
fn test_build_root_directly() {
    let leaves = CbmtLeaves {
        nodes: vec![
            int32_to_node(2),
            int32_to_node(3),
            int32_to_node(5),
            int32_to_node(7),
            int32_to_node(11),
        ],
    };
    let root = cbmt_build_merkle_root(&leaves, node_merge).expect("Failed to build root");
    println!("root: {}", node_to_int32(&root));
    // Expected root value for these leaves is 4.
    assert_eq!(node_to_int32(&root), 4);
    println!("test_build_root_directly passed.");
}

#[test]
fn test_rebuild_proof() {
    let leaves = CbmtLeaves {
        nodes: vec![
            int32_to_node(2),
            int32_to_node(3),
            int32_to_node(5),
            int32_to_node(7),
            int32_to_node(11),
        ],
    };
    let mut tree = CbmtTree {
        nodes: Vec::new(),
        length: 0,
        capacity: 0,
    };
    let ret = cbmt_build_merkle_tree(&mut tree, &leaves, node_merge);
    assert_eq!(ret, 0);
    let root = cbmt_tree_root(&tree);
    // Build a proof for leaves with original indices 0 and 3.
    let leaf_indices = CbmtIndices {
        values: vec![0, 3],
        capacity: 0,
    };
    let proof = cbmt_tree_build_proof(&tree, &leaf_indices).expect("Failed to build proof");
    println!(
        "proof.indices: {:?}, proof.lemmas: {:?}",
        proof.indices.values,
        proof
            .lemmas
            .iter()
            .map(|n| node_to_int32(n))
            .collect::<Vec<_>>()
    );
    // Expected: proof.indices should be [4, 7] (i.e. leaves offset by 4) and lemmas [11, 2]
    assert_eq!(proof.indices.values, vec![4, 7]);
    assert_eq!(proof.lemmas.len(), 2);
    assert_eq!(node_to_int32(&proof.lemmas[0]), 11);
    assert_eq!(node_to_int32(&proof.lemmas[1]), 2);

    // Verify the proof against the tree root.
    let ret = cbmt_proof_verify(&proof, &root, &leaves, node_merge);
    assert_eq!(ret, 0);
    println!("test_rebuild_proof passed.");
}

#[test]
fn test_build_proof() {
    // This test is left empty (as in the original C file).
    println!("test_build_proof passed.");
}
pub fn main(){}