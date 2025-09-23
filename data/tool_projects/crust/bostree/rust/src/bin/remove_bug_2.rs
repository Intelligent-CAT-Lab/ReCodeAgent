use std::process;

use Bostree::bostree::*;
use Bostree::*;

fn cmp(a: &str, b: &str) -> i32 {
    a.cmp(b) as i32
}

/// Create a test tree by inserting keys from 'A' to 'Y'.
fn test_tree() -> BOSTree {
    let mut tree = BOSTree::bostree_new(cmp, None);
    for c in 'A'..'Z' {
        let key = c.to_string();
        let _node = tree.bostree_insert(key, None);
    }
    tree
}

#[test]
fn test() {
    let mut tree = test_tree();
    // Remove the node with key "G"
    if let Some(node) = tree.bostree_lookup("G") {
        tree.bostree_remove(&node);
    }
    // Remove the node with key "H"
    if let Some(node) = tree.bostree_lookup("H") {
        tree.bostree_remove(&node);
    }

    test_tree_sanity(&tree);
    if tree.bostree_lookup("E").is_none() {
        println!("Nodes missing after removing another one");
        process::exit(1);
    }
}
fn main() {}